import { useSandpack } from "@codesandbox/sandpack-react";
import { limoo_eval } from "../../pkg/limoo";

const TerminalViewer = () => {
    const { sandpack } = useSandpack();
    const { files, activePath } = sandpack;

    const code = files[activePath].code;
    return <pre style={{ width: "50%" }}>{limoo_eval(code)}</pre>;
};

export default TerminalViewer;
