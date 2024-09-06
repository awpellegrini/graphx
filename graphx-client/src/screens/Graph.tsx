import GraphComponent from '../components/Graph/Graph';
import useGraphxGraph from '../hooks/useGraphxGraph';
import GraphSettings from '../components/Graph/components/GraphSettings';
import GraphAdjMatrix from '../components/Graph/components/GraphAdjMatrix';

type GraphScreenProps = {
  type?: 'example' | 'random';
  onBack: () => void;
};

export default function GraphScreen({type, onBack}: GraphScreenProps) {
  const {
    graph,
    directed,
    adj_mat,
    customActives,
    customSelected,
    getGraphRandom,
    getSubGraph,
  } = useGraphxGraph(type);

  return (
    <div>
      <button
        style={{position: 'fixed', top: 0, left: 0, zIndex: 9}}
        onClick={onBack}
      >
        back
      </button>

      <div style={{position: 'fixed', bottom: 20, left: 20, zIndex: 9}}>
        <GraphSettings
          vertices_num={graph.vertices.length}
          // half the number of edges for undirected graph
          edges_num={graph.edges.length / (directed ? 1 : 2)}
          directed={directed}
          onChange={getGraphRandom}
        />
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

      <GraphComponent
        nodes={graph.vertices}
        edges={graph.edges}
        directed={directed}
        onNodeClick={({id}) => getSubGraph(id)}
        labelType="nodes"
        selections={customSelected}
        actives={customActives}
      />
    </div>
  );
}
