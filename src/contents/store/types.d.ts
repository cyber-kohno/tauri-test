export type Store = {
  phase: Phase;
  scanRequest: ScanRequest;
  resultTree?: UsableNode;

  rvReq: RevalidateRequest;

  preview?: Preview;
};

export type Phase = 'listup' | 'revalidate' | 'execute';

export type ScanRequest = {
  rootPath: string;
  expectedDepth: number;
  limitDepth?: number;
  dirConds: DirCond[];
  fileConds: FileCond[];
};

export interface FileCond {
  pattern: string;
  isExclusion: boolean;
}
export interface DirCond extends FileCond {
  depth?: number;
}

export type ScanResponse = {
  result: string;
  node: PayloadNode;
};
export interface PayloadNode {
  name: string;
  children: null | PayloadNode[];
}

export interface ChildProps {
  fileCnt: number;
  selectCnt: number;
  isOpen: boolean;
  nodes: UsableNode[];
}
export interface UsableNode {
  name: string;
  isSelected: boolean;
  path: string;
  child?: ChildProps;
}

export type NodeIndent = "none" | "middle" | "last";
export type NodeDispProps = {
  indents: NodeIndent[];
  node: UsableNode;
  seq: number;
};

export type Preview = {
  src: string;
  language: string;
};


export type RevalidateRequest = {
  targets: string;
};
