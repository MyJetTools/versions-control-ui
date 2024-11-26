

### Settings Example

```yaml
envs:
  common:
    url: ssh:root@10.0.0.1:22->http://127.0.0.1:8083
    host: sub-dobmain1.domain.com
  wallet-app:
    url: ssh:root@10.0.0.1:22->http://127.0.0.1:8083
    host: sub-dobmain2.domain.com

ssh_pass_key_promt: true
ssh_private_key_path: ~/.ssh/id_rsa

```