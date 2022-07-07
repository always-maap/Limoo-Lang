import { PlayIcon } from '@radix-ui/react-icons';
import { Button } from './Button';

type Props = {
  examples: { name: string; code: string }[];
  onChangeExample(index: number): void;
};

export default function Examples(props: Props) {
  const { examples, onChangeExample } = props;

  return (
    <div>
      {examples.map((example, index) => (
        <div key={example.name} className="relative my-6">
          <div dangerouslySetInnerHTML={{ __html: example.code }} />

          <Button
            className="absolute right-0 bottom-0"
            onClick={() => {
              onChangeExample(index);
            }}
          >
            RUN <PlayIcon />
          </Button>
        </div>
      ))}
    </div>
  );
}
