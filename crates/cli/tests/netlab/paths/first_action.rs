//! First foreground read after a quiet interval, before any status query.
use super::*;

use serde_json::{Value, json};

const REVISION: &str = "first_foreground_after_idle_v1";

async fn capture(node: &LabNode, epoch: Instant) -> Value {
    node.with(move |_| async move {
        let started = epoch.elapsed().as_micros();
        let mut commands = serde_json::Map::new();
        for (name, program, args) in [
            ("interfaces", "ip", vec!["-s", "-j", "link", "show"]),
            ("qdisc_json", "tc", vec!["-s", "-j", "qdisc", "show"]),
            ("qdisc_text", "tc", vec!["-s", "-d", "qdisc", "show"]),
        ] {
            let began = epoch.elapsed().as_micros();
            let mut command = tokio::process::Command::new(program);
            command.kill_on_drop(true).args(args);
            let value = match tokio::time::timeout(Duration::from_secs(2), command.output()).await {
                Ok(Ok(output)) => json!({"ok":output.status.success(),"timed_out":false,
                    "status":output.status.code(),"stdout":String::from_utf8_lossy(&output.stdout),
                    "stderr":String::from_utf8_lossy(&output.stderr)}),
                Ok(Err(error)) => json!({"ok":false,"timed_out":false,"error":error.to_string()}),
                Err(_) => {
                    json!({"ok":false,"timed_out":true,"error":"first-action snapshot deadline"})
                }
            };
            commands.insert(
                name.into(),
                json!({"started_us":began,
                "ended_us":epoch.elapsed().as_micros(),"command":value}),
            );
        }
        json!({"started_us":started,"ended_us":epoch.elapsed().as_micros(),"commands":commands})
    })
    .await
}

async fn snapshots(pair: &SpeedPair, epoch: Instant) -> Value {
    let (a, b) = tokio::join!(capture(&pair.a, epoch), capture(&pair.b, epoch));
    json!({"alpha":a,"beta":b})
}

fn captures_ok(value: &Value) -> bool {
    ["alpha", "beta"].iter().all(|side| {
        ["interfaces", "qdisc_json", "qdisc_text"]
            .iter()
            .all(|name| value[*side]["commands"][*name]["command"]["ok"] == true)
    })
}

async fn first(s: &str, reverse: bool, phase_s: u64) {
    let pair = speed_pair(s, 300).await;
    let epoch = Instant::now();
    let mut prepared_paths = serde_json::Map::new();
    for (a, b) in [(&pair.a, &pair.b), (&pair.b, &pair.a)] {
        let (_, link) = a
            .until(b, Duration::from_secs(25), |link| link.hop == Hop::Relay)
            .await
            .expect("the relay carries before the quiet interval");
        prepared_paths.insert(
            a.name.clone(),
            json!({"hop":format!("{:?}",link.hop),"raw":format!("{link:?}"),
                "observed_us":epoch.elapsed().as_micros()}),
        );
    }
    let before_idle = snapshots(&pair, epoch).await;
    let quiet_started = epoch.elapsed().as_micros();
    tokio::time::sleep(Duration::from_secs(10)).await;
    let restore_started = epoch.elapsed().as_micros();
    delay(&pair.a.dev, "eth1", 0, LinkDirection::Both).await;
    let restore_ended = epoch.elapsed().as_micros();
    tokio::time::sleep(Duration::from_secs(phase_s)).await;
    // Only read namespace counters here. No Node query or opposite-end call
    // may refresh the idle wire before this first foreground operation.
    let before_action = snapshots(&pair, epoch).await;
    let (from, to) = if reverse {
        (&pair.b, &pair.a)
    } else {
        (&pair.a, &pair.b)
    };
    let peer = to.name.clone();
    let request_started = epoch.elapsed().as_micros();
    let (took, error) = crate::attempts::measure(
        s,
        &from.id,
        &to.name,
        Duration::from_secs(30),
        from.with(move |node| async move { node.ls_of(&peer, "/tmp").await.map(|_| ()) }),
    )
    .await;
    let request_ended = epoch.elapsed().as_micros();
    let after_action = snapshots(&pair, epoch).await;
    // Path reads happen after the cost interval, never before the operation.
    let (a, b) = tokio::join!(pair.a.link(&pair.b), pair.b.link(&pair.a));
    let road = |link: Option<khor_core::Link>| {
        link.map(|link| json!({"hop":format!("{:?}",link.hop),"raw":format!("{link:?}")}))
    };
    let ok = error.is_none()
        && captures_ok(&before_idle)
        && captures_ok(&before_action)
        && captures_ok(&after_action);
    eprintln!(
        "[netlab-first-action] {}",
        json!({"schema":1,"revision":REVISION,"case":s,
            "from":from.name,"to":to.name,"from_id":from.id,"to_id":to.id,"phase_s":phase_s,
            "direct_before_ms":300,"direct_restored_ms":0,"relay_edge_ms":40,
            "quiet_ms":10000,"caller_ms":30000,"diagnostic":"off",
            "prepared_paths":prepared_paths,
            "quiet_started_us":quiet_started,"restore_started_us":restore_started,
            "restore_ended_us":restore_ended,"request_started_us":request_started,
            "request_ended_us":request_ended,"caller_elapsed_us":took.as_micros(),
            "error":error,"functional_ok":ok,
            "before_idle":before_idle,"before_action":before_action,"after_action":after_action,
            "paths_after":{"alpha":road(a),"beta":road(b)},
            "paths_observed_us":epoch.elapsed().as_micros(),
            "full_N1_accepted":false}),
    );
    result(
        s,
        ok,
        format!(
            "phase {phase_s}s, {} -> {}; first read {took:?}, error {error:?}; measurement only",
            from.name, to.name
        ),
    );
    assert!(ok, "first read and namespace captures completed");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "isolated first foreground read and wire counters"]
async fn p8_alpha_phase0() {
    first("p8-alpha-0", false, 0).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "isolated first foreground read and wire counters"]
async fn p8_alpha_phase5() {
    first("p8-alpha-5", false, 5).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "isolated first foreground read and wire counters"]
async fn p8_alpha_phase14() {
    first("p8-alpha-14", false, 14).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "isolated first foreground read and wire counters"]
async fn p8_beta_phase0() {
    first("p8-beta-0", true, 0).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "isolated first foreground read and wire counters"]
async fn p8_beta_phase5() {
    first("p8-beta-5", true, 5).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "isolated first foreground read and wire counters"]
async fn p8_beta_phase14() {
    first("p8-beta-14", true, 14).await;
}
