import Container from '../components/Container';
import TerminalViewer from '../components/TerminalViewer';
import CodeEditor from '../components/CodeEditor';
import Nav from '../components/Nav';
import { useState } from 'react';
import Examples from '../components/Examples';

type Props = {
  examples: typeof rawExamples;
};

export default function Home(props: Props) {
  const { examples } = props;
  const [example, setExample] = useState(rawExamples[1].code);

  function onChangeExample(index: number) {
    setExample(rawExamples[index].code);
  }

  function onRunClick(code: string) {
    console.log(code);
    setExample(code);
  }

  return (
    <Container>
      <Nav />

      <div className="flex justify-between border-2 rounded">
        <CodeEditor code={example} onRunClick={onRunClick} />
        <TerminalViewer code={example} />
      </div>

      <Examples examples={examples} onChangeExample={onChangeExample} />
    </Container>
  );
}

const rawExamples = [
  {
    name: 'hello_world',
    code: `print("Hello, world!")`,
  },
  {
    name: 'fibonacci',
    code: `let fibonacci = fn(n) {
  if (n < 2) {
    return n;
  }
  
  return fibonacci(n - 1) + fibonacci(n - 2);
}

fibonacci(10)`,
  },
  {
    name: 'linear_search',
    code: `let linear_search = fn(arr, target) {
  let i = 0;
  while(i < len(arr)) {
    if(arr[i] == target) {
      return i;
    }
    i = i + 1;
  }
}
 
linear_search([1, 2, 3], 2);`,
  },
];

export async function getStaticProps() {
  const shiki = await import('shiki');
  const highlighter = await shiki.getHighlighter({
    theme: 'min-light',
  });

  const examples = rawExamples.map(({ name, code }) => {
    return {
      name,
      code: highlighter.codeToHtml(code, { lang: 'rust' }),
    };
  });

  return {
    props: {
      examples,
    },
  };
}
