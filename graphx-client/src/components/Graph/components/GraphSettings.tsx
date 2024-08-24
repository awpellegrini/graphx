type GraphSettingsProps = {
  vertices_num: number;
  edges_num: number;
  onChange: (value: {vertices_num: number; edges_num: number}) => void;
};

export default function GraphSettings(settings: GraphSettingsProps) {
  return (
    <div>
      <div>
        <label>Number of vertices</label>
        <input
          type="number"
          placeholder="4"
          min={2}
          value={settings.vertices_num || 4}
          onChange={(e) => {
            settings.onChange({
              vertices_num: parseInt(e.target.value),
              edges_num: settings.edges_num,
            });
          }}
        />
      </div>

      <div>
        <label>Number of edges</label>
        <input
          type="number"
          placeholder="4"
          min={2}
          value={settings.edges_num || 3}
          onChange={(e) => {
            console.log(e.target.value);
            settings.onChange({
              vertices_num: settings.vertices_num,
              edges_num: parseInt(e.target.value),
            });
          }}
        />
      </div>
    </div>
  );
}
