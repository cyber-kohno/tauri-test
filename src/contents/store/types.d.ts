export type Store = {
  scanRequest: ScanRequest;
};

export type ScanRequest = {
  rootPath: string;
  expectedDepth: number;
  limitDepth?: number;
  dirConds: DirCond[];
};

export type DirCond = {
  pattern: string;
  depth?: number;
  isExclusion: boolean;
};
