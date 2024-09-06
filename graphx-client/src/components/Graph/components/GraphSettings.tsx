type GraphSettingsValues = {
  vertices_num: number;
  edges_num: number;
  directed: boolean;
};

type GraphSettingsProps = GraphSettingsValues & {
  onChange: (value: GraphSettingsValues) => void;
  onChangeDirect?: (value: boolean) => void;
};

export default function GraphSettings(settings: GraphSettingsProps) {
  return (
    <div style={containerStyle}>
      <div style={checkboxStyle}>
        <label htmlFor="directed" style={labelStyle}>
          Directed
          <input
            id="directed"
            type="checkbox"
            checked={settings.directed ? true : false}
            onChange={(e) => {
              settings.onChange({
                vertices_num: settings.vertices_num,
                edges_num: settings.edges_num,
                directed: e.target.checked || false,
              });
            }}
          />
        </label>
      </div>

      <div style={rowStyle}>
        <label>Vertices</label>
        <input
          type="number"
          placeholder="4"
          min={2}
          value={settings.vertices_num}
          onChange={(e) => {
            settings.onChange({
              vertices_num: parseInt(e.target.value),
              edges_num: settings.edges_num,
              directed: settings.directed,
            });
          }}
          style={inputStyle}
        />
      </div>

      <div style={rowStyle}>
        <label>Edges</label>
        <input
          type="number"
          placeholder="4"
          min={2}
          value={settings.edges_num}
          onChange={(e) => {
            settings.onChange({
              vertices_num: settings.vertices_num,
              edges_num: parseInt(e.target.value),
              directed: settings.directed,
            });
          }}
          style={inputStyle}
        />
      </div>
    </div>
  );
}

const containerStyle = {
  width: 120,
  padding: '4px 8px',
  border: '1px solid white',
};

const checkboxStyle = {
  marginBottom: 10,
};
const labelStyle = {
  display: 'flex',
  alignItems: 'center', // center vertically
  gap: '2.5rem', // distance between checkbox and text
};

const rowStyle = {
  display: 'flex',
  justifyContent: 'space-between',
  alignItems: 'center',
};

const inputStyle = {
  width: 40,
};
