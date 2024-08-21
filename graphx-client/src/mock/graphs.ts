import {GraphEdge, GraphNode} from 'reagraph';

export const SIMPLE_NODES: GraphNode[] = [
  {
    id: '1',
    label: '1',
  },
  {
    id: '2',
    label: '2',
  },
  {
    id: '3',
    label: '3',
  },
  {
    id: '4',
    label: '4',
  },
  {
    id: '5',
    label: '5',
  },
  {
    id: '6',
    label: '6',
  },
  {
    id: '7',
    label: '7',
  },
];

export const SIMPLE_EDGES: GraphEdge[] = [
  {
    id: '1-1',
    source: '1',
    target: '2',
    label: '1 --> 2',
    size: 3,
  },
  {
    id: '2-3',
    source: '2',
    target: '3',
    label: '2 --> 3',
    size: 5,
  },
  {
    id: '3-1',
    source: '3',
    target: '1',
    label: '3 --> 1',
    size: 7,
  },

  {
    id: '4-5',
    source: '4',
    target: '5',
    label: '4 --> 5',
  },
  {
    id: '4-6',
    source: '4',
    target: '6',
    label: '4 --> 6',
  },
  {
    id: '5-7',
    source: '5',
    target: '7',
    label: '5 --> 7',
  },

  {
    id: '5-2',
    source: '5',
    target: '2',
    label: '5 --> 2',
  },
];

const nodes = [
  {id: '1', label: 'Tesla', data: {category: 'EV'}},
  {id: '2', label: 'C8', data: {category: 'ICE'}},
];

export const COMPLEX: {nodes: GraphNode[]; edges: GraphEdge[]} = {
  nodes,
  edges: [],
};
