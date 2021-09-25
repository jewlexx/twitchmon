import { Client, ChatUserstate, SubUserstate, Badges } from 'tmi.js';
import { program } from 'commander';
import figlet from 'figlet';
import chalk from 'chalk';
import dayjs from 'dayjs';

program.version('{VERSION}');
program.option('-u, --user <user>', 'Twitch chat to monitor');

const bigText = (text: string) =>
  new Promise<string | undefined>((resolve, reject) => {
    figlet(text, (err, data) => {
      if (err) {
        reject(err);
      }
      resolve(data);
    });
  });

const getBadges = (badges: Badges | undefined) => {
  const array = [];

  if (badges?.broadcaster) {
    array.push(chalk.red('[BR]'));
  }
  if (badges?.admin) {
    array.push(chalk.magentaBright('[A]'));
  }
  if (badges?.subscriber) {
    array.push(chalk.cyanBright('[S]'));
  }
  if (badges?.vip) {
    array.push(chalk.redBright('[V]'));
  }
  if (badges?.moderator) {
    array.push(chalk.greenBright('[M]'));
  }
  if (badges?.partner) {
    array.push(chalk.magenta('[P]'));
  }
  if (badges?.founder) {
    array.push(chalk.yellow('[F]'));
  }
  if (badges?.['bits-leader']) {
    array.push(chalk.green('[BL]'));
  } else if (badges?.bits) {
    array.push(chalk.green(`[B${badges.bits}]`));
  }
  if (badges?.turbo) {
    array.push(chalk.cyan('[T]'));
  }
  if (badges?.['sub-gifter']) {
    array.push(chalk.green('[SG]'));
  }
  if (badges?.staff) {
    array.push(chalk.yellow('[ST]'));
  }
  if (badges?.global_mod) {
    array.push(chalk.magenta('[GM]'));
  }

  return array.join('');
};

const getUserName = (tags: ChatUserstate | SubUserstate) => {
  const displayName = tags['display-name'] || tags.username;
  const color = chalk.hex(tags.color || '#fff');
  const badges = getBadges(tags.badges);

  return `${badges} ${color(displayName)}`;
};

const getChannelName = async (user: string | undefined) => {
  if (!user) {
    const prompt = (await import('inquirer')).prompt;
    const { user } = await prompt({
      type: 'input',
      name: 'user',
      message: 'Twitch chat to monitor',
    });
    return user;
  }
  return user;
};

program.parseAsync(process.argv).then(async (cmd) => {
  const opts = cmd.opts();
  const user = await getChannelName(opts.user);

  const client = new Client({
    channels: [user],
  });

  await client.connect().catch(console.error);
  console.log(await bigText(user));

  dayjs.extend((await import('dayjs/plugin/duration')).default);
  dayjs.extend((await import('dayjs/plugin/relativeTime')).default);

  client.on('message', (_channel, tags, message, _self) => {
    console.log(`${getUserName(tags)}: ${message}`);
  });

  client.on('subscription', (_channel, _username, method, message, tags) => {
    const type = method.prime ? 'primed' : 'subscribed';

    console.log(`\n${getUserName(tags)} ${type}: ${message}\n`);
  });

  client.on('cheer', (_channel, tags, message) => {
    console.log(`${getUserName(tags)} cheered: ${message}`);
  });

  client.on('emoteonly', (_channel, enabled) => {
    if (enabled) {
      console.log(`\nChat now in emote only\n`);
    } else {
      console.log(`\nChat no longer in emote only\n`);
    }
  });

  client.on('followersonly', async (_channel, enabled, length) => {
    if (enabled) {
      const time = dayjs.duration(length, 'minutes').humanize();
      console.log(`\nChat now in ${time} followers only \n`);
    } else {
      console.log(`\nChat no longer in followers only\n`);
    }
  });

  client.on('slowmode', async (_channel, enabled, length) => {
    if (enabled) {
      const time = dayjs.duration(length, 'seconds').humanize();
      console.log(`\nChat now in ${time} slow mode \n`);
    } else {
      console.log(`\nChat no longer in slow mode\n`);
    }
  });

  client.on('clearchat', () => {
    console.clear();
    console.log('\nChat cleared\n');
  });
});
