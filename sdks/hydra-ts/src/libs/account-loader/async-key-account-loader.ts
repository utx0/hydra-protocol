import { Commitment, PublicKey } from "@solana/web3.js";
import { Ctx } from "../../types";
import { from } from "rxjs";
import { mergeMap } from "rxjs/operators";

import { Parser, IAccountLoader } from "./types";
import { KeyOrGetter } from "./index";
import { InternalAccountLoader } from "./internal-account-loader";
import { cache } from "./cache";

const loaderStore = new Map<string, IAccountLoader<any>>();

// AccountLoader
// Wrapper to handle all the issues arrising from requiring async keys
export function AsyncKeyAccountLoader<T>(
  _ctx: Ctx,
  getter: KeyOrGetter,
  accountParser: Parser<T>
): IAccountLoader<T> {
  let _key: PublicKey | undefined = undefined;
  let _accountLoader: IAccountLoader<T> | undefined;

  async function key() {
    if (typeof _key !== "undefined") {
      return _key;
    }

    if (typeof getter === "function") {
      _key = await getter();
    } else {
      _key = getter;
    }

    return _key;
  }

  async function ready() {
    await getAccountLoader();
  }

  async function getAccountLoader() {
    if (typeof _accountLoader !== "undefined") {
      return _accountLoader;
    }

    const __key = await key();
    _accountLoader = cache(loaderStore, __key, () =>
      InternalAccountLoader(_ctx, __key, accountParser)
    );
    return _accountLoader!;
  }

  function stream(commitment?: Commitment) {
    return from(getAccountLoader()).pipe(
      mergeMap((loader) => loader.stream(commitment))
    );
  }

  function ctx() {
    return _ctx;
  }

  async function info(commitment?: Commitment) {
    return (await getAccountLoader()).info(commitment);
  }

  async function isInitialized() {
    return (await getAccountLoader()).isInitialized();
  }
  function parser() {
    return accountParser;
  }

  function onChange(callback: (info: T) => void, commitment?: Commitment) {
    return stream(commitment).subscribe(
      (info) => info && callback(info.account.data)
    ).unsubscribe;
  }

  return {
    key,
    ctx,
    info,
    ready,
    isInitialized,
    parser,
    stream,
    onChange,
  };
}
