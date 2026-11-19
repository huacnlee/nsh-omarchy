// Line, area, bar and pie charts; Tab to one, Left and Right step its points.
import { View } from "nsh";
import { AreaChart, BarChart, LineChart, PieChart } from "nsh-omarchy";
import { Demos, type Family, Section } from "./Section";

const months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
const desktop = [186, 305, 237, 73, 209, 214];
const mobile = [80, 200, 120, 190, 130, 140];
const visits = months.map((label, i) => ({ label, value: desktop[i] }));
const browsers = [
  { label: "Chrome", value: 275 },
  { label: "Safari", value: 200 },
  { label: "Firefox", value: 187 },
  { label: "Edge", value: 173 },
  { label: "Other", value: 90 },
];

function LineChartPage() {
  return (
    <Demos>
      <Section title="Two series" description="A natural curve; Tab to the chart, Left and Right step the points.">
        <LineChart
          data={visits}
          name="Desktop"
          series={[{ name: "Mobile", values: mobile }]}
          curve="natural"
          height={160}
          accessibilityLabel="Visitors, January to June"
        />
      </Section>
    </Demos>
  );
}

function AreaChartPage() {
  return (
    <Demos>
      <Section title="AreaChart" description="A line with its area filled; values carry a unit.">
        <View class="w-[480px]">
          <AreaChart data={visits} name="Desktop" height={160} unit=" visits" />
        </View>
      </Section>
    </Demos>
  );
}

function BarChartPage() {
  return (
    <Demos>
      <Section title="Stacked" description="Two series stacked in each bar.">
        <View class="w-[480px]">
          <BarChart data={visits} name="Desktop" series={[{ name: "Mobile", values: mobile }]} stacked height={160} />
        </View>
      </Section>
    </Demos>
  );
}

function PieChartPage() {
  return (
    <Demos>
      <Section title="Donut" description="Parts of a whole, with a legend of shares.">
        <PieChart data={browsers} donut={0.6} size={160} accessibilityLabel="Visitors by browser" />
      </Section>
    </Demos>
  );
}

export const charts: Family = {
  title: "Charts",
  pages: [
    { value: "line-chart", title: "LineChart", page: LineChartPage },
    { value: "area-chart", title: "AreaChart", page: AreaChartPage },
    { value: "bar-chart", title: "BarChart", page: BarChartPage },
    { value: "pie-chart", title: "PieChart", page: PieChartPage },
  ],
};
