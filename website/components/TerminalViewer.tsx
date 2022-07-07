import dynamic from 'next/dynamic';

type Props = { code: string };

const TerminalViewer = dynamic({
  loader: async () => {
    const limoo = await import('../../pkg/');

    // eslint-disable-next-line react/display-name
    return (props: Props) => {
      const { code } = props;

      return <pre style={{ width: '50%' }}>{limoo.limoo_eval(code)}</pre>;
    };
  },
});

export default TerminalViewer;
