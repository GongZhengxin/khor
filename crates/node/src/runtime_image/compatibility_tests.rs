//! Published MessagePack shapes, independent of today's enum ordering.
use super::*;
use crate::proto::{self, Request, Response};

// These are the deployed rc20 shapes. Deliberately keep a different order
// from the current enums: MessagePack words must be names, not ordinals.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum PublishedRequest {
    Ls { path: String },
    Sessions,
}
#[derive(Debug, Serialize, Deserialize)]
enum PublishedResponse {
    SessionRows { rows: Vec<khor_core::Session> },
    Refused { why: String },
}

#[test]
fn published_directory_and_session_words_keep_their_actual_messagepack_bytes() {
    let published = proto::encode(&PublishedRequest::Sessions).unwrap();
    assert_eq!(published, b"\xa8Sessions");
    assert_eq!(
        proto::encode_request(&Request::Sessions).unwrap(),
        published
    );
    assert!(matches!(
        proto::decode::<Request>(&published),
        Ok(Request::Sessions)
    ));

    let published = PublishedRequest::Ls {
        path: "/fixture".into(),
    };
    let bytes = proto::encode_request(&Request::Ls {
        path: "/fixture".into(),
    })
    .unwrap();
    assert_eq!(bytes, proto::encode(&published).unwrap());
    assert_eq!(
        proto::decode::<PublishedRequest>(&bytes).unwrap(),
        published
    );
    assert!(
        matches!(proto::decode::<Request>(&bytes), Ok(Request::Ls { path }) if path == "/fixture")
    );

    let published = PublishedResponse::Refused {
        why: "fixture refusal".into(),
    };
    let bytes = proto::encode_response(&Response::Refused {
        why: "fixture refusal".into(),
    })
    .unwrap();
    assert_eq!(bytes, proto::encode(&published).unwrap());
    assert!(
        matches!(proto::decode::<PublishedResponse>(&bytes).unwrap(), PublishedResponse::Refused { why } if why == "fixture refusal")
    );
    assert!(
        matches!(proto::decode::<Response>(&bytes), Ok(Response::Refused { why }) if why == "fixture refusal")
    );

    let bytes = proto::encode_response(&Response::SessionRows { rows: vec![] }).unwrap();
    assert!(
        matches!(proto::decode::<PublishedResponse>(&bytes).unwrap(), PublishedResponse::SessionRows { rows } if rows.is_empty())
    );
}

#[test]
fn an_import_word_is_refused_by_published_decoders_without_becoming_an_old_operation() {
    let request = Request::SessionImport {
        home: khor_core::DeviceId([42; 32]),
        action: khor_core::session_import::SessionImportAction::Read,
    };
    let bytes = proto::encode_request(&request).unwrap();
    let why = proto::decode::<PublishedRequest>(&bytes).unwrap_err();
    assert!(why.contains("SessionImport"), "{why}");
    assert!(matches!(
        proto::decode::<Request>(&bytes),
        Ok(Request::SessionImport { .. })
    ));
    let bytes = proto::encode_response(&Response::SessionImport {
        value: khor_core::session_import::SessionImportReply::Imported {
            home: "fixture".into(),
            id: "fixture-import".into(),
            intent: khor_core::session_import::ImportIntent::Import,
        },
    })
    .unwrap();
    assert!(
        proto::decode::<PublishedResponse>(&bytes)
            .unwrap_err()
            .contains("SessionImport")
    );
    assert!(matches!(
        proto::decode::<Response>(&bytes),
        Ok(Response::SessionImport { .. })
    ));
}

#[test]
fn audited_import_addition_preserves_the_published_runtime_node_contract() {
    let actual = current().unwrap().release.contract.node;
    assert_eq!(
        actual, "khor/0:sha256:7bdf9a8cedcc26b783f3a6714899df875cbcb429e2f1ee64aa5a7d065e75fcda",
        "published live residents must remain eligible during upgrade"
    );
}
