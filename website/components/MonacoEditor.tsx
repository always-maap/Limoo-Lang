import Editor from '@monaco-editor/react';
import { useActiveCode, SandpackStack, FileTabs, useSandpack } from '@codesandbox/sandpack-react';
import '@codesandbox/sandpack-react/dist/index.css';
import { useState } from 'react';

function MonacoEditor() {
  const { code, updateCode } = useActiveCode();
  const { sandpack } = useSandpack();
  const [val, setVal] = useState('');

  return (
    <SandpackStack customStyle={{ height: '500px', margin: 0, position: 'relative' }}>
      <FileTabs />
      <div style={{ flex: 1, paddingTop: 8, background: '#1e1e1e' }}>
        <Editor
          width="100%"
          height="500px"
          theme="vs-dark"
          key={sandpack.activePath}
          defaultValue={code}
          onChange={(value) => {
            setVal(value);
          }}
        />
        <button
          style={{ position: 'absolute', bottom: 0, left: 0, color: 'white' }}
          onClick={() => updateCode(val)}
        >
          run
        </button>
      </div>
    </SandpackStack>
  );
}

export default MonacoEditor;
