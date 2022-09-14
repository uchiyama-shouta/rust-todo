import Button from "./Button";

import type { ComponentMeta, ComponentStoryObj } from "@storybook/react";

type T = typeof Button;
type Story = ComponentStoryObj<T>;

export default {
  title: "Button",
  component: Button,
  args: { children: "送信する", onClick: () => alert("clicked!") },
} as ComponentMeta<T>;

export const Default: Story = {};
