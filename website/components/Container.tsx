import { FC } from "react";

type Props = {
    className?: string;
};

const Container: FC<Props> = (props) => {
    const { children, className, ...rest } = props;

    return (
        <div className={`container mx-auto px-4 ${className}`} {...rest}>
            {children}
        </div>
    );
};

export default Container;
