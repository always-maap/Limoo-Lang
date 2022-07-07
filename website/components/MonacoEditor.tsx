import Editor from '@monaco-editor/react';
import { useActiveCode, SandpackStack, FileTabs, useSandpack } from '@codesandbox/sandpack-react';
import '@codesandbox/sandpack-react/dist/index.css';
import { useState } from 'react';
import { Button } from './Button';
import { PlayIcon } from '@radix-ui/react-icons';

function MonacoEditor() {
  const { code, updateCode } = useActiveCode();
  const { sandpack } = useSandpack();
  const [val, setVal] = useState('');

  return (
    <SandpackStack customStyle={{ height: '500px', margin: 0, position: 'relative' }}>
      <FileTabs />
      <div style={{ flex: 1, paddingTop: 8 }}>
        <Editor
          width="100%"
          height="500px"
          theme="light"
          key={sandpack.activePath}
          defaultValue={code}
          onChange={(value) => {
            setVal(value);
          }}
        />
        <Button onClick={() => updateCode(val)}>
          RUN <PlayIcon />
        </Button>
      </div>
    </SandpackStack>
  );
}

export default MonacoEditor;
