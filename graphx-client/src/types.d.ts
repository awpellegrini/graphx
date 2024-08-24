type GraphxVertex = {
  id: string;
  label: string;
};

type GraphxEdge = {
  id: string;
  source: string;
  target: string;
  label: string;
  size: number;
};

type GraphxGraph = {
  vertices: GraphxVertex[];
  edges: GraphxEdge[];
};
