import { HTMLAttributes, ReactNode } from 'react';
import cx from 'classnames';

type Props = {
  children?: ReactNode;
} & HTMLAttributes<HTMLButtonElement>;

export function Button(props: Props) {
  const { children, ...rest } = props;
  return (
    <button
      {...rest}
      className={cx(
        'bg-yellow-500 text-white text-lg font-bold bg-yellow-500 hover:bg-yellow-400 focus:outline-none text-white font-semibold h-10 px-4 rounded-lg w-full flex items-center justify-center w-auto gap-2',
        rest.className
      )}
    >
      {children}
    </button>
  );
}
