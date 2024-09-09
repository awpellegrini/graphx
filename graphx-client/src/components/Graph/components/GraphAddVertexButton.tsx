import {useCallback, useState} from 'react';

type GraphAddVertexButtonProps = {
  onSubmit: (name: string) => void;
};

export default function GraphAddVertexButton({
  onSubmit,
}: GraphAddVertexButtonProps) {
  const [isOpen, setIsOpen] = useState(false);
  const [name, setName] = useState('');

  const handleClose = useCallback(() => {
    setIsOpen(false);
    setName('');
  }, []);

  const handleSubmit = useCallback(() => {
    const trimmedName = name.replace(/\s/g, '_');
    onSubmit(trimmedName);
    setIsOpen(false);
    setName('');
  }, [onSubmit, name]);

  return (
    <div style={containerStyle}>
      {isOpen ? (
        <div style={{display: 'flex'}}>
          <input
            style={inputStyle}
            type="text"
            placeholder="Vertex Name"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />

          <button onClick={handleSubmit}>Submit</button>

          <div onClick={handleClose} style={closeButtonStyle}>
            Close
          </div>
        </div>
      ) : (
        <button onClick={() => setIsOpen(!isOpen)}>Add Vertex</button>
      )}
    </div>
  );
}

const containerStyle = {
  display: 'flex',
  justifyContent: 'center',
  height: 60,
  flex: 1,
};

const closeButtonStyle = {
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
  width: 60,
  height: 60,
  cursor: 'pointer',
  textDecoration: 'underline',
};

const inputStyle = {
  flex: 1,
  fontSize: 20,
  height: 60,
  margin: 0,
  padding: 0,
  paddingLeft: 10,
  marginRight: 20,
};
