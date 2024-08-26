import GraphComponent from '../components/Graph/Graph';
import useGraphxGraph from '../hooks/useGraphxGraph';
import GraphSettings from '../components/Graph/components/GraphSettings';
import GraphAdjMatrix from '../components/Graph/components/GraphAdjMatrix';

type GraphScreenProps = {
  type?: 'example' | 'random';
  onBack: () => void;
};

export default function GraphScreen({type, onBack}: GraphScreenProps) {
  const {graph, adj_mat, customActives, getGraphRandom, getSubGraph} =
    useGraphxGraph(type);

  return (
    <div>
      <button
        style={{position: 'fixed', top: 0, left: 0, zIndex: 9}}
        onClick={onBack}
      >
        back
      </button>

      {/* <button
        style={{position: 'fixed', top: 0, right: 0, zIndex: 9}}
        onClick={onBack}
      >
        cycle
      </button> */}

      <div style={{position: 'fixed', bottom: 0, left: 0, zIndex: 9}}>
        <GraphSettings
          vertices_num={graph.vertices.length}
          edges_num={graph.edges.length}
          onChange={getGraphRandom}
        />
      </div>

      <div
        style={{
          position: 'fixed',
          bottom: 20,
          right: 20,
          zIndex: 9,
          // width: 220,
          // height: 220,
          maxWidth: 180,
          maxHeight: 180,
          overflow: 'auto',
          border: '1px solid red',
          // display: 'flex',
          // justifyContent: 'bottom',
          // alignItems: 'right',
        }}
      >
        <GraphAdjMatrix adj_mat={adj_mat} />
      </div>
      <GraphComponent
        nodes={graph.vertices}
        edges={graph.edges}
        onEdgeClick={(a) => {
          console.log('click', a);
        }}
        onNodeClick={(a, b) => {
          console.log('click', a, b);
          getSubGraph(a.id);
        }}
        actives={customActives}
      />
    </div>
  );
}
