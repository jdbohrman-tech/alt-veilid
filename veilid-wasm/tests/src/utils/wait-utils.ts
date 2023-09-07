export const waitForMs = (milliseconds: number) => {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
};
