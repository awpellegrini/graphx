import {useCallback, useEffect, useState} from 'react';
import {
  GenerateGraphRandomOptions,
  GraphResponse,
  generateGraphExample,
  generateGraphRandom,
  getConnectedSubGraph,
} from '../api';

const DEFAULT_GRAPH_DATA = {graph: {vertices: [], edges: []}, adj_mat: []};

export default function useGraphxGraph(type: 'example' | 'random') {
  const [graphData, setGraphData] = useState<GraphResponse>(DEFAULT_GRAPH_DATA);
  const [directed, setDirected] = useState(true);

  const [customActives, setCustomActives] = useState<string[]>([]);
  const [customSelected, setCustomSelected] = useState<string[]>([]);

  const getGraphExample = useCallback(async () => {
    const data = await generateGraphExample();
    setGraphData(data);
  }, []);

  const getGraphRandom = useCallback(
    async (options: GenerateGraphRandomOptions) => {
      const data = await generateGraphRandom(options);
      setDirected(options.directed || false);
      setGraphData(data);
    },
    []
  );

  const getSubGraph = useCallback(async (vertex_id: string) => {
    setCustomSelected([vertex_id]);
    const data = await getConnectedSubGraph(vertex_id);

    const [vertex_ids, edges] = data;

    let edge_ids = edges.map(([a, b]) => `${a} ${b}`);

    setCustomActives([...vertex_ids, ...edge_ids]);
  }, []);

  useEffect(() => {
    if (type === 'example') {
      getGraphExample();
    }

    if (type === 'random') {
      getGraphRandom({vertices_num: 4, edges_num: 3});
    }
  }, [type, getGraphExample, getGraphRandom]);

  return {
    graph: graphData.graph,
    directed: directed,
    adj_mat: graphData.adj_mat,
    customActives,
    customSelected,
    getGraphRandom,
    getSubGraph,
  };
}
