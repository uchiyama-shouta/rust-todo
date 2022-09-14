import type { ComponentProps, FC } from "react";

type Props = ComponentProps<"button">;

const Button: FC<Props> = (props) => {
  return <button className="text-lg text-red-500" {...props} />;
};

export default Button;
