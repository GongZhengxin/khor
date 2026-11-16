import { useEffect, useRef, useState } from "react";
import { agentFleet, type AgentFleet, type FleetAgent } from "@/api";
import { RegionState } from "@/components/RegionState";
import { cli, gui } from "@/gen/catalog";
import { MoreSection } from "./MorePage";

/** The one word a row's state wears. */
const stateWord = (row: FleetAgent): string =>
  row.program ? cli.agents_fleet_installed : cli.agents_fleet_absent;

/**
 * Where a row's program is, then where its settings are.
 *
 * Two different facts, and a tool that was uninstalled leaves the second
 * behind — so the path a person is looking for is the first one, and the
 * settings directory is only shown when the program is not there to show.
 */
const where = (row: FleetAgent): string =>
  row.program ?? (row.config ? `${row.config}` : "—");

/**
 * **What this machine has, not what khor knows about** (docs/WINDOWS…,
 * `khor agents fleet`): each known agent, whether it is installed here,
 * the road khor would take to it, and what that road can do.
 *
 * A row that is not installed stays on the list: it is the answer to
 * "what could I install", and hiding it would make the page a report on
 * this machine alone rather than a map of what to do next.
 */
export function AgentFleetSection() {
  const [fleet, setFleet] = useState<AgentFleet | null>(null);
  const [error, setError] = useState<string | null>(null);
  const alive = useRef(true);
  useEffect(() => {
    alive.current = true;
    agentFleet().then(
      (answer) => {
        if (alive.current) setFleet(answer);
      },
      (why) => {
        if (alive.current) setError(String(why));
      },
    );
    return () => {
      alive.current = false;
    };
  }, []);
  return (
    <MoreSection name="agent-fleet" label={gui.agent_fleet}>
      <p className="px-row py-in text-callout text-muted-foreground">{gui.agent_fleet_hint}</p>
      {error !== null ? (
        <RegionState state="error" message={error} />
      ) : fleet === null ? (
        <RegionState state="loading-never" />
      ) : (
        <>
          {fleet.runner === null && (
            <p className="px-row py-in text-callout text-muted-foreground">
              {cli.agents_fleet_no_runner}
            </p>
          )}
          {fleet.agents.map((row) => (
            <div
              key={row.id}
              data-setting-row="fact"
              data-setting-row-name={`fleet-${row.id}`}
              data-fleet-agent={row.id}
              data-fleet-state={row.program ? "installed" : "absent"}
              className="flex flex-col gap-0 px-row py-in"
            >
              <span className="flex flex-wrap items-baseline gap-x-in text-body">
                <span>{row.name}</span>
                <span className="text-callout text-secondary-foreground">{stateWord(row)}</span>
                {row.road && <span className="text-callout text-muted-foreground">{row.road}</span>}
              </span>
              <span className="break-words font-mono text-caption text-muted-foreground">
                {where(row)}
              </span>
              {row.capabilities.length > 0 && (
                <span className="text-caption text-muted-foreground">
                  {row.capabilities.join(" · ")}
                </span>
              )}
            </div>
          ))}
        </>
      )}
    </MoreSection>
  );
}
