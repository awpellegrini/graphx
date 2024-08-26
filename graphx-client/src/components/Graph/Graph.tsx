import {
  GraphCanvas,
  GraphCanvasProps,
  GraphCanvasRef,
  GraphEdge,
  GraphNode,
  darkTheme,
  useSelection,
} from 'reagraph';
import {useRef} from 'react';

type GraphProps = GraphCanvasProps & {
  nodes: GraphNode[];
  edges: GraphEdge[];
};

export default function Graph({nodes, edges, ...options}: GraphProps) {
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
      theme={darkTheme}
      animated={false}
      // edgeLabelPosition="natural"
      labelType="all"
      // for clustering byt category
      // clusterAttribute="category"
      {...options}
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
