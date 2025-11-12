import { writable } from "svelte/store";
import type { ScanRequest, Store } from "./types";

const store = writable<Store>({
  scanRequest: {
    rootPath: "",
    expectedDepth: 0,
    limitDepth: 0,
    dirConds: [],
  },
});

export type StoreUtil = {
  lastStore: Store;
  commit: () => void;
};

export const createStoreUtil = (lastStore: Store): StoreUtil => {
  return {
    lastStore,
    // commit: () => store.set({ ...lastStore }),
    commit: () => store.set(lastStore),
  };
};

export default store;
