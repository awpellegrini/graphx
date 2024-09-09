/* eslint-disable camelcase */
import {invoke} from '@tauri-apps/api';
import {convert} from '../utils/conversion';

//examples
export type GraphResponse = {
  graph: GraphxGraph;
  adj_mat: number[][];
};

export const generateGraphExample = async (): Promise<GraphResponse> => {
  const response: string = await invoke('generate_graph_example');
  const parsed = JSON.parse(response);

  return {
    graph: convert(parsed.graph),
    adj_mat: parsed.adj_mat,
  };
};

export type GenerateGraphRandomOptions = {
  vertices_num?: number;
  edges_num?: number;
  directed?: boolean;
};

export const generateGraphRandom = async (
  options: GenerateGraphRandomOptions
): Promise<GraphResponse> => {
  const response: any = await invoke('generate_graph_random', {
    vertices: options.vertices_num,
    edges: options.edges_num,
    directed: options.directed || false,
  });

  const parsed = JSON.parse(response);

  return {
    graph: convert(parsed.graph),
    adj_mat: parsed.adj_mat,
  };
};

export const getConnectedSubGraph = async (
  vertex: string
): Promise<[vertices: string[], edges: [string, string]]> => {
  const response: any = await invoke('get_connected_subgraph', {vertex});

  return JSON.parse(response);
};

export const getPlaygroundGraph = async (): Promise<GraphResponse> => {
  const response: any = await invoke('get_graph');
  const parsed = JSON.parse(response);

  return {
    graph: convert(parsed.graph),
    adj_mat: parsed.adj_mat,
  };
};

export const createVertex = async (label: string): Promise<GraphResponse> => {
  const response: any = await invoke('create_vertex', {label});
  const parsed = JSON.parse(response);

  return {
    graph: convert(parsed.graph),
    adj_mat: parsed.adj_mat,
  };
};

export const create_edge = async (
  outid: string,
  inid: string,
  label: string
): Promise<GraphResponse> => {
  const response: any = await invoke('create_edge', {outid, inid, label});
  const parsed = JSON.parse(response);

  return {
    graph: convert(parsed.graph),
    adj_mat: parsed.adj_mat,
  };
};

export const delete_vertex = async (id: string): Promise<GraphResponse> => {
  const response: any = await invoke('delete_vertex', {id});
  const parsed = JSON.parse(response);

  return {
    graph: convert(parsed.graph),
    adj_mat: parsed.adj_mat,
  };
};

export const delete_edge = async (id: string): Promise<GraphResponse> => {
  const response: any = await invoke('delete_edge', {id});
  const parsed = JSON.parse(response);

  return {
    graph: convert(parsed.graph),
    adj_mat: parsed.adj_mat,
  };
};
