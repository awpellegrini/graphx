import {Graph, GraphNode} from 'reagraph';
import GraphComponent from '../components/Graph';
import {COMPLEX, SIMPLE_EDGES, SIMPLE_NODES} from '../mock/graphs';

const background = {
  width: '100vw',
  height: '100vh',
  // background: 'black',
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
};

export default function GraphScreen() {
  console.log('SIMPLE_NODES', SIMPLE_NODES);
  return (
    <div>
      <GraphComponent nodes={SIMPLE_NODES} edges={SIMPLE_EDGES} />
    </div>
  );
}
