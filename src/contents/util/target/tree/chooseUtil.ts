import type {
  NodeDispProps,
  NodeIndent,
  UsableNode,
} from "../../../store/types";

namespace ChooseUtil {
  export const getDispRecords = (
    root: UsableNode,
    isFlat: boolean,
  ): NodeDispProps[] => {
    return !isFlat ? buildTreeChooser(root) : buildFlatChooser(root);
  };

  const buildTreeChooser = (root: UsableNode): NodeDispProps[] => {
    const list: NodeDispProps[] = [];

    const rec = (
      node: UsableNode,
      indents: NodeIndent[],
      isOpen: boolean,
    ): [number, number] => {
      if (isOpen) {
        list.push({
          indents,
          node,
          seq: list.length,
        });
      }
      let [fileCnt, selectCnt] = [0, 0];
      if (node.child != undefined) {
        const child = node.child;
        [child.fileCnt, child.selectCnt] = [0, 0];
        const nodes = node.child.nodes;
        nodes.forEach((n, i) => {
          const nextIndents: NodeIndent[] = indents.slice();
          // 自身がlastの場合、子要素はnoneにする
          if (nextIndents[nextIndents.length - 1] === "last")
            nextIndents[nextIndents.length - 1] = "none";
          nextIndents.push(
            (() => {
              if (i === nodes.length - 1) return "last";
              else return "middle";
            })(),
          );
          const [cFileCnt, cSelectCnt] = rec(
            n,
            nextIndents,
            isOpen && child.isOpen,
          );
          child.fileCnt += cFileCnt;
          child.selectCnt += cSelectCnt;
        });
        fileCnt += child.fileCnt;
        selectCnt += child.selectCnt;
      } else {
        fileCnt++;
        selectCnt += node.isSelected ? 1 : 0;
      }
      return [fileCnt, selectCnt];
    };
    rec(root, [], true);

    return list;
  };
  const buildFlatChooser = (root: UsableNode) => {
    const list: NodeDispProps[] = [];

    const rec = (node: UsableNode) => exec(node);
    const exec = (node: UsableNode) => {
      if (node.child == undefined) {
        list.push({
          indents: [],
          node,
          seq: list.length,
        });
      } else {
        node.child.nodes.forEach((n) => exec(n));
      }
    };
    if (root.child == undefined) throw new Error();

    root.child.nodes.forEach((n) => exec(n));
    return list;
  };
}
export default ChooseUtil;
