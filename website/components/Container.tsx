import { ReactNode } from 'react';

type Props = {
  className?: string;
  children?: ReactNode;
};

export default function Container(props: Props) {
  const { children, className, ...rest } = props;

  return (
    <div className={`max-w-7xl mx-auto px-4 sm:px-6 md:px-8 ${className}`} {...rest}>
      {children}
    </div>
  );
}
