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
  directed?: boolean;
};

export default function Graph({
  nodes,
  edges,
  directed,
  ...options
}: GraphProps) {
  const graphRef = useRef<GraphCanvasRef | null>(null);
  const selections = useSelection({
    ref: graphRef,
    nodes: nodes,
    edges: edges,
    pathSelectionType: 'all',
    actives: options.actives,
    selections: options.selections,
  });

  return (
    <GraphCanvas
      ref={graphRef}
      nodes={nodes}
      edges={edges}
      theme={darkTheme}
      animated={false}
      draggable={true}
      edgeArrowPosition={directed ? 'end' : 'none'}
      {...selections}
      {...options}
    />
  );
}
