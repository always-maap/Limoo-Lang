import { FC } from 'react';

type Props = {
  className?: string;
};

const Container: FC<Props> = (props) => {
  const { children, className, ...rest } = props;

  return (
    <div className={`max-w-7xl mx-auto px-4 sm:px-6 md:px-8 ${className}`} {...rest}>
      {children}
    </div>
  );
};

export default Container;
