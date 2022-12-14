import { AccountInfo, Commitment, PublicKey } from "@solana/web3.js";
import { Ctx } from "../../types";
import { Parser, IAccountLoader, AccountData } from "./types";
import { concat, from, Observable, share, tap } from "rxjs";
import { cache } from "./cache";

// TODO: maintain a list of streams by public key to avoid setting up too many streams
const streamStore = new Map<string, Observable<any>>();

// InternalAccountLoader
// Returns an account loader that is already initialized with a key
export function InternalAccountLoader<T>(
  _ctx: Ctx,
  _key: PublicKey,
  accountParser: Parser<T>
): IAccountLoader<T> {
  async function key() {
    return _key;
  }

  async function info(commitment?: Commitment) {
    let info = await _ctx.connection.getAccountInfo(_key, commitment);
    if (info === null) {
      throw new Error("info couldnt be fetched for " + _key.toString());
    }

    return { ...info, data: accountParser(info) };
  }

  async function isInitialized() {
    try {
      const inf = await info();
      return !!inf;
    } catch (err) {
      return false;
    }
  }

  async function getAccountData(
    commitment?: Commitment
  ): Promise<AccountData<T> | undefined> {
    let account: AccountInfo<T>;
    try {
      account = await info(commitment);
    } catch (err) {
      return;
    }

    return {
      account,
      pubkey: _key,
    };
  }

  function stream(commitment?: Commitment) {
    return cache(streamStore, _key, () => {
      // first send current data then changes
      const currentData$ = from(getAccountData(commitment));
      const changes$ = new Observable<AccountData<T>>((subscriber) => {
        // Listen for account change events
        // Send events to stream
        const id = _ctx.connection.onAccountChange(
          _key,
          (rawAccount: AccountInfo<Buffer> | null) => {
            if (rawAccount) {
              const account = {
                ...rawAccount,
                data: accountParser(rawAccount),
              };
              subscriber.next({ pubkey: _key, account });
            } else {
              subscriber.next();
            }
          },
          commitment
        );

        return () => {
          _ctx.connection.removeAccountChangeListener(id);
        };
      });
      return concat(currentData$, changes$);
    });
  }

  function onChange(callback: (info: T) => void, commitment: Commitment) {
    return stream(commitment).subscribe(
      (info) => info && callback(info.account.data)
    ).unsubscribe;
  }

  function ready() {
    return Promise.resolve();
  }
  function parser() {
    return accountParser;
  }
  function ctx() {
    return _ctx;
  }

  return {
    key,
    info,
    isInitialized,
    onChange,
    stream,
    ready,
    parser,
    ctx,
  };
}
