import { FC } from "react";

import { TypingArea } from "widgets";

interface TypingPageProps {
  textId?: number;
}

export const TypingPage: FC<TypingPageProps> = ({ textId }) => {
  return (
    <section>
      <TypingArea />
    </section>
  );
};
