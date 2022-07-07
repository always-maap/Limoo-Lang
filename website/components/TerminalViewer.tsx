import dynamic from 'next/dynamic';

type Props = { code: string };

const TerminalViewer = dynamic({
  loader: async () => {
    const limoo = await import('../../pkg/');

    // eslint-disable-next-line react/display-name
    return (props: Props) => {
      const { code } = props;

      return <pre className="w-1/2 p-1 border-l-2">{limoo.limoo_eval(code)}</pre>;
    };
  },
});

export default TerminalViewer;
