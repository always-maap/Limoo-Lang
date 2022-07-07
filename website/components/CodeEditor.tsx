import { useCallback, useRef } from 'react';
import { Button } from './Button';
import { PlayIcon } from '@radix-ui/react-icons';
import Editor from '@uiw/react-codemirror';
import { rust } from '@codemirror/lang-rust';

type Props = {
  code: string;
  onRunClick(code: string): void;
};

export default function CodeEditor(props: Props) {
  const { code, onRunClick } = props;
  const editorValue = useRef('');

  const onChange = useCallback((value) => {
    editorValue.current = value;
  }, []);

  return (
    <div className="w-full relative">
      <Editor extensions={[rust()]} value={code} onChange={onChange} height="500px" spellCheck={false} />
      <Button
        className="absolute right-0 bottom-0"
        onClick={() => {
          onRunClick(editorValue.current);
        }}
      >
        RUN <PlayIcon />
      </Button>
    </div>
  );
}
