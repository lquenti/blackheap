import KaTeX from "katex";

type FormulaProps = {
  tex: string;
};

const Formula = ({ tex }: FormulaProps) => {
  const html = KaTeX.renderToString(tex, {
    displayMode: true,
    output: "mathml",
  });
  return <span dangerouslySetInnerHTML={{ __html: html }} />;
};

export default Formula;
