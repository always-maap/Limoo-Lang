import { PlayIcon } from '@radix-ui/react-icons';
import { Button } from './Button';

type Props = {
  examples: { name: string; code: string }[];
  onChangeExample(index: number): void;
};

export default function Examples(props: Props) {
  const { examples, onChangeExample } = props;

  return (
    <div className="grid lg:grid-cols-2 gap-4 my-2">
      {examples.map((example, index) => (
        <div key={example.name} className="w-full relative mt-12 tb-6 border-2 rounded p-4">
          <div
            className="absolute left-8 top-0 translate-y-[-100%] text-neutral-700 bg-gray-100 px-2 py-2"
            style={{ borderRadius: '8px 8px 0px 0px' }}
          >
            {example.name}.lp
          </div>
          <div dangerouslySetInnerHTML={{ __html: example.code }} />
          <Button
            className="absolute bottom-2 right-2"
            onClick={() => {
              onChangeExample(index);
            }}
          >
            RUN <PlayIcon width={21} height={21} />
          </Button>
        </div>
      ))}
    </div>
  );
}
