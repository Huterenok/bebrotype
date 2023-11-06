import { FC } from "react";

import { TextInfo } from "widgets";

interface TextPageProps {
  id: number;
}

export const TextPage: FC<TextPageProps> = async ({ id }) => {
  return <TextInfo id={id} />;
};
