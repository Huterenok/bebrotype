import { FC } from "react";

import { TextList, getAllTexts } from "enities/Text";

export const TextsPage: FC = async () => {
  const texts = await getAllTexts(5);

  return (
    <section>
      Users texts
      <TextList texts={texts} />
    </section>
  );
};
