import { useSandpack } from '@codesandbox/sandpack-react';
import dynamic from 'next/dynamic';

const TerminalViewer = dynamic({
  loader: async () => {
    const limoo = await import('../../pkg/');

    // eslint-disable-next-line react/display-name
    return () => {
      const { sandpack } = useSandpack();
      const { files, activePath } = sandpack;

      const code = files[activePath].code;

      return <pre style={{ width: '50%' }}>{limoo.limoo_eval(code)}</pre>;
    };
  },
});

export default TerminalViewer;
