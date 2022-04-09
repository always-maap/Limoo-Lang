import { SandpackProvider, SandpackLayout } from '@codesandbox/sandpack-react';
import '@codesandbox/sandpack-react/dist/index.css';
import Container from '../components/Container';
import TerminalViewer from '../components/TerminalViewer';
import MonacoEditor from '../components/MonacoEditor';

const exampleCode = `let sum = fn(a, b) {
  return a + b;
}

sum(5, 2)`;

export default function Home() {
  return (
    <Container>
      <SandpackProvider
        customSetup={{
          entry: 'main.lp',
          files: {
            'main.lp': {
              code: exampleCode,
            },
          },
        }}
      >
        <SandpackLayout theme="dark">
          <MonacoEditor />
          <TerminalViewer />
        </SandpackLayout>
      </SandpackProvider>
    </Container>
  );
}
