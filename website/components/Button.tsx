import { HTMLAttributes, ReactNode } from 'react';

type Props = {
  children?: ReactNode;
} & HTMLAttributes<HTMLButtonElement>;

export function Button(props: Props) {
  const { children, ...rest } = props;
  return (
    <button
      className="bg-yellow-500 text-white text-lg font-bold bg-yellow-600 hover:bg-yellow-500 focus:outline-none text-white font-semibold h-10 px-6 rounded-lg w-full flex items-center justify-center w-auto gap-2"
      style={{ position: 'absolute', bottom: 5, left: 5 }}
      {...rest}
    >
      {children}
    </button>
  );
}
