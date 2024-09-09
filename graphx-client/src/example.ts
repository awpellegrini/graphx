import {GraphEdge, GraphNode} from 'reagraph';

export const EXAMPLE_VERTICES: GraphNode[] = [
  {id: '1', label: 'Rome', data: {category: 'IT'}, fill: 'green', size: 8},
  {id: '2', label: 'London', data: {category: 'UK'}},
  {id: '3', label: 'Detroit', data: {category: 'USA'}},
  {id: '4', label: 'Los Angeles', data: {category: 'USA'}},
];

export const EXAMPLE_EDGES: GraphEdge[] = [
  {
    id: '1',
    source: '2',
    target: '1',
    label: 'London - Rome',
    subLabel: '1000 km',
  },

  {
    id: '2',
    source: '3',
    target: '1',
    label: 'Detroit - Rome',
    size: 6,
  },
  {
    id: '3',
    source: '4',
    target: '3',
    label: 'Los Angeles - Detroit',
  },
];
