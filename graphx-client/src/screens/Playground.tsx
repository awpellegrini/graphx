import GraphComponent from '../components/Graph/Graph';
import GraphSettings from '../components/Graph/components/GraphSettings';
import GraphAdjMatrix from '../components/Graph/components/GraphAdjMatrix';
import GraphAddVertexButton from '../components/Graph/components/GraphAddVertexButton';
import usePlayground from '../hooks/usePlayground';
import {
  ContextMenuEvent,
  InternalGraphNode,
  InternalGraphEdge,
  RadialMenu,
} from 'reagraph';

type GraphScreenProps = {
  onBack: () => void;
};

function isEdge(
  pet: InternalGraphNode | InternalGraphEdge
): pet is InternalGraphEdge {
  return (pet as InternalGraphEdge).source !== undefined;
}

export default function PlaygroundScreen({onBack}: GraphScreenProps) {
  const {
    graph,
    adj_mat,
    customActives,
    customSelected,
    addVertex,
    deleteVertex,
    deleteEdge,
    getSubGraph,
    handleAddEdges,
    clearSelections,
  } = usePlayground();

  const renderContextMenu = ({data, onClose}: ContextMenuEvent) => {
    let options: any = [];
    if (isEdge(data)) {
      options = [
        {
          label: 'Remove Edge',
          onClick: () => {
            deleteEdge(data.id);
            onClose();
          },
        },
      ];

      return <RadialMenu onClose={onClose} items={options} />;
    } else {
      options = [
        {
          label: 'Connect Node',
          onClick: () => {
            handleAddEdges(data.id);
            onClose();
          },
        },
        {
          label: 'Delete Node',
          onClick: () => {
            deleteVertex(data.id);
            onClose();
          },
        },
      ];
      return <RadialMenu onClose={onClose} items={options} />;
    }
  };
  return (
    <div>
      <button
        style={{position: 'fixed', top: 20, left: 20, zIndex: 9}}
        onClick={onBack}
      >
        back
      </button>

      <div style={{position: 'fixed', top: 20, right: 20, zIndex: 9}}>
        <GraphAddVertexButton onSubmit={addVertex} />
      </div>

      <GraphAdjMatrix
        adj_mat={adj_mat}
        style={{
          position: 'fixed',
          bottom: 20,
          right: 20,
          zIndex: 9,
        }}
      />

      <div style={{position: 'fixed', bottom: 20, left: 20, zIndex: 9}}>
        <GraphSettings
          disabled
          vertices_num={graph.vertices.length}
          // half the number of edges for undirected graph
          edges_num={graph.edges.length / (true ? 1 : 2)}
          directed={true}
          onChange={() => {}}
        />
      </div>

      <GraphComponent
        nodes={graph.vertices}
        edges={graph.edges}
        directed={true}
        onNodeClick={() => {}}
        onNodeDoubleClick={({id}) => getSubGraph(id)}
        onCanvasClick={clearSelections}
        labelType="nodes"
        selections={customSelected}
        actives={customActives}
        contextMenu={renderContextMenu}
      />
    </div>
  );
}
