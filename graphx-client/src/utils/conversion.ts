export const convert = (graph: any) => {
  let vertices = graph.vertices.map((v: any) => ({
    id: v.id,
    label: v.label,
  }));

  let edges = graph.edges.map((e: any) => ({
    id: `${e.outbound_id} ${e.inbound_id}`,
    label: e.label,
    source: e.outbound_id,
    target: e.inbound_id,
  }));

  return {vertices, edges};
};
