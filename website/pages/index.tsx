import { SandpackProvider, SandpackLayout, SandpackCodeEditor } from '@codesandbox/sandpack-react';
import '@codesandbox/sandpack-react/dist/index.css';
import Container from '../components/Container';
import TerminalViewer from '../components/TerminalViewer';

export default function Home() {
  return (
    <Container>
      <SandpackProvider
        customSetup={{
          entry: 'main.lp',
          files: { 'main.lp': { code: 'let x = 5;' } },
        }}
      >
        <SandpackLayout>
          <SandpackCodeEditor id="editor" showTabs showLineNumbers />
          <TerminalViewer />
        </SandpackLayout>
      </SandpackProvider>
    </Container>
  );
}
