export type Store = {
  scanRequest: ScanRequest;
  resultTree?: UsableNode;
};

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
  node: Node;
};
export interface Node {
  name: string;
  children: null | Node[];
}

export interface UsableNode {
  name: string;
  isOpen: boolean;
  isSelected: boolean;
  children: null | UsableNode[];
}

export type NodeDispProps = {
  indent: number;
  str: string;
  node: UsableNode;
};
