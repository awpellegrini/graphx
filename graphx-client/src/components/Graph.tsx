import {
  GraphCanvas,
  GraphCanvasRef,
  GraphEdge,
  GraphNode,
  useSelection,
} from 'reagraph';
import {useRef} from 'react';

const fixedactives = ['2', '1', '3', '1-1', '2-3', '3-1'];
// ["2"] (1)

type GraphProps = {
  nodes: GraphNode[];
  edges: GraphEdge[];
};

export default function Graph({nodes, edges}: GraphProps) {
  console.log('nodes', nodes);
  const graphRef = useRef<GraphCanvasRef | null>(null);
  const {selections, actives, onNodeClick, onCanvasClick} = useSelection({
    ref: graphRef,
    nodes: nodes,
    edges: edges,
    pathSelectionType: 'all',
  });

  return (
    <GraphCanvas
      ref={graphRef}
      nodes={nodes}
      edges={edges}
      selections={selections}
      actives={actives}
      // selections={[]}
      // actives={fixedactives}
      onCanvasClick={onCanvasClick}
      onNodeClick={onNodeClick}
      //
      animated={false}
      edgeLabelPosition="natural"
      labelType="edges"
      // for clustering byt category
      // clusterAttribute="category"
    />
  );

  // return (
  //   <GraphCanvas
  //     animated={false}
  //     // onNodeDoubleClick={(node) => alert(node.label)}
  //     nodes={COMPLEX.nodes}
  //     edges={COMPLEX.edges}
  //   />
  // );
}
