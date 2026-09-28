// Minimal off-chain event listener stub for the Marketplace contract.
//
// Once you've deployed this contract to testnet and it's emitting events
// (see the TODOs in ../src/lib.rs), this script polls the RPC `getEvents`
// endpoint and prints each one. This is the "indexer" half of
// 16-events-indexer -- a real indexer would write these into a database
// instead of console.log-ing them.
//
// Usage:
//   node listen.mjs <contractId>
//
// You'll need the `@stellar/stellar-sdk` package (see
// ../../../guessing-game-tutorial/package.json for a working example of a
// project that already depends on it).

import { rpc } from "@stellar/stellar-sdk";

const RPC_URL = "https://soroban-testnet.stellar.org";
const POLL_INTERVAL_MS = 5000;

async function main() {
  const contractId = process.argv[2];
  if (!contractId) {
    console.error("Usage: node listen.mjs <contractId>");
    process.exit(1);
  }

  const server = new rpc.Server(RPC_URL);

  // TODO: get a real starting ledger instead of hardcoding one -- e.g. via
  // server.getLatestLedger() minus a small buffer, or a checkpoint you
  // persist between runs.
  let startLedger = 0; // TODO: replace with a real ledger number

  console.log(`Watching events for ${contractId} from ledger ${startLedger}...`);

  setInterval(async () => {
    // TODO: call server.getEvents({ startLedger, filters: [{ type: "contract",
    // contractIds: [contractId] }] }), log each event's topic/value, and
    // advance startLedger to (lastEvent.ledger + 1) so you don't reprocess
    // the same events on the next poll.
  }, POLL_INTERVAL_MS);
}

main();
