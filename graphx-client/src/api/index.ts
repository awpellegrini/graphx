/* eslint-disable camelcase */
import {invoke} from '@tauri-apps/api';

export const testConnection = async (): Promise<string> => {
  const response: any = await invoke('testconnect');
  console.log(response);

  return response;
};

export type GraphResponse = {
  graph: GraphxGraph;
  adj_mat: number[][];
};

export const generateGraphExample = async (): Promise<GraphResponse> => {
  const response: any = await invoke('generate_graph_example');
  const parsed = JSON.parse(response);

  let graphobj = eval('(' + parsed + ')');
  return graphobj as GraphResponse;
};

export type GenerateGraphRandomOptions = {
  vertices_num?: number;
  edges_num?: number;
};

export const generateGraphRandom = async (
  options: GenerateGraphRandomOptions
): Promise<GraphResponse> => {
  const response: any = await invoke('generate_graph_random', {
    vertices: options.vertices_num,
    edges: options.edges_num,
  });
  const parsed = JSON.parse(response);

  let graphobj = eval('(' + parsed + ')');
  return graphobj as GraphResponse;
};

export const getConnectedSubGraph = async (
  vertex: string
): Promise<GraphResponse> => {
  const response: any = await invoke('get_connected_subgraph', {vertex});
  const parsed = JSON.parse(response);

  let graphobj = eval('(' + parsed + ')');
  return graphobj as GraphResponse;
};
