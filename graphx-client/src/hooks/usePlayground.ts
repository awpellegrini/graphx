import {useCallback, useEffect, useMemo, useState} from 'react';
import {
  GraphResponse,
  getPlaygroundGraph,
  createVertex,
  getConnectedSubGraph,
  delete_vertex,
  create_edge,
  delete_edge,
} from '../api';

const DEFAULT_GRAPH_DATA = {graph: {vertices: [], edges: []}, adj_mat: []};

export default function usePlayground() {
  const [graphData, setGraphData] = useState<GraphResponse>(DEFAULT_GRAPH_DATA);

  const [customActives, setCustomActives] = useState<string[]>([]);
  const [customSelected, setCustomSelected] = useState<string[]>([]);

  const [selectedId, setSelectedId] = useState<string>('');

  const getGraph = useCallback(async () => {
    const data = await getPlaygroundGraph();
    setGraphData(data);
  }, []);

  const addVertex = useCallback(async (name: string) => {
    const data = await createVertex(name);
    setGraphData(data);
  }, []);

  const deleteVertex = useCallback(async (id: string) => {
    const data = await delete_vertex(id);
    setGraphData(data);
  }, []);

  const deleteEdge = useCallback(async (id: string) => {
    const data = await delete_edge(id);
    setGraphData(data);
  }, []);

  const getSubGraph = useCallback(async (vertex_id: string) => {
    setCustomSelected([vertex_id]);
    const [vertex_ids, edges] = await getConnectedSubGraph(vertex_id);
    let edge_ids = edges.map(([a, b]) => `${a} ${b}`);

    setCustomActives([...vertex_ids, ...edge_ids]);
  }, []);

  const handleAddEdges = useCallback(
    async (edge_id: string) => {
      console.log({edge_id});
      if (selectedId) {
        const data = await create_edge(selectedId, edge_id, 'edge');
        setGraphData(data);
        setSelectedId('');
      } else {
        setSelectedId(edge_id);
      }
    },
    [selectedId]
  );

  useEffect(() => {
    getGraph();
  }, [getGraph]);

  const graph = useMemo(() => {
    let v = graphData.graph.vertices.map((v) => ({...v}));
    if (selectedId) {
      const index = v.findIndex((vertex) => vertex.id === selectedId);
      //@ts-ignore
      v[index].fill = 'red';
    }
    return {vertices: v, edges: graphData.graph.edges};
  }, [graphData, selectedId]);

  console.log({selectedId});
  return {
    graph: graph,
    adj_mat: graphData.adj_mat,
    customActives,
    customSelected,
    getGraph,
    addVertex,
    deleteVertex,
    deleteEdge,
    getSubGraph,
    handleAddEdges,
    clearSelectedId: () => setSelectedId(''),
    clearSelections: () => {
      setCustomActives([]);
      setCustomSelected([]);
    },
  };
}
