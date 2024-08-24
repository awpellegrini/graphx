import {useCallback, useEffect, useState} from 'react';
import {
  GenerateGraphRandomOptions,
  GraphResponse,
  generateGraphExample,
  generateGraphRandom,
  getConnectedSubGraph,
} from '../api';

const DEFAULT_GRAPH_DATA = {graph: {vertices: [], edges: []}, adj_mat: []};

type type = 'example' | 'random';

export default function useGraphxGraph(type?: type) {
  const [graphData, setGraphData] = useState<GraphResponse>(DEFAULT_GRAPH_DATA);

  const getGraphExample = useCallback(async () => {
    const data = await generateGraphExample();
    setGraphData(data);
  }, []);

  const getGraphRandom = useCallback(
    async (options: GenerateGraphRandomOptions) => {
      const data = await generateGraphRandom(options);
      setGraphData(data);
    },
    []
  );

  const getSubGraph = useCallback(async (vertex_id: string) => {
    const data = await getConnectedSubGraph(vertex_id);
    console.log({data});
    // setGraphData(data);
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
    adj_mat: graphData.adj_mat,
    getGraphRandom,
    getSubGraph,
  };
}
